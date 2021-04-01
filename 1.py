#!/usr/bin/env python

import argparse
import queue
import sys
from functools import partial
from pathlib import Path

import soundfile
import torch
import torch.nn.functional as F
import tritonclient.grpc as grpcclient
from tritonclient.utils import InferenceServerException


class UserData:
    def __init__(self):
        self._completed_requests = queue.Queue()


# Define the callback function. Note the last two parameters should be
# result and error. InferenceServerClient would povide the results of an
# inference as grpcclient.InferResult in result. For successful
# inference, error will be None, otherwise it will be an object of
# tritonclientutils.InferenceServerException holding the error details
def callback(user_data, result, error):
    if error:
        user_data._completed_requests.put(error)
    else:
        user_data._completed_requests.put(result)


def async_stream_send(
        triton_client, values, sequence_id, model_name, model_version, topk
):

    count = 1
    for signal, num_of_samples in values:
        # Create the tensor for INPUT
        inputs = []
        print("PCM shape =", signal.shape)
        inputs.append(grpcclient.InferInput("PCM", signal.shape, "INT16"))
        print("NUM_OF_SAMPLES shape =", num_of_samples.shape)
        inputs.append(
            grpcclient.InferInput("NUM_OF_SAMPLES", num_of_samples.shape, "INT32")
        )
        # Initialize the data
        inputs[0].set_data_from_numpy(signal)
        print("Signal: ", signal)
        inputs[1].set_data_from_numpy(num_of_samples)
        print("num_of_samples: ", num_of_samples)
        print(inputs)
        print("#######################################################################################################")
        outputs = []
        outputs.append(
            grpcclient.InferRequestedOutput("DECODER_LOGITS", class_count=topk)
        )
        # Issue the asynchronous sequence inference.
        triton_client.async_stream_infer(
            model_name=model_name,
            inputs=inputs,
            outputs=outputs,
            request_id="{}_{}".format(sequence_id, count),
            sequence_id=sequence_id,
            sequence_start=(count == 1),
            sequence_end=(count == len(values)),
        )
        count = count + 1


if __name__ == "__main__":
    parser = argparse.ArgumentParser(description="Test triton server")
    parser.add_argument("--wav", type=Path, default=Path("output.wav"))
    parser.add_argument("--url", default="localhost:8001")
    parser.add_argument("--model-name", default="encdeclanglabel")
    parser.add_argument("--model-version", type=int, default=-1)
    parser.add_argument("--stream_timeout", type=int, default=10)
    parser.add_argument("--topk", type=int, default=1)
    parser.add_argument("--verbose", action="store_true")
    FLAGS = parser.parse_args()

    with soundfile.SoundFile(FLAGS.wav, "r") as sf:
        audio = sf.read(dtype="int16")

    num_samples = audio.shape[0]
    signal = torch.unsqueeze(torch.as_tensor(audio), 0)
    padded = F.pad(signal, (0, 80000 - signal.shape[1]))

    num_of_samples = torch.unsqueeze(
        torch.unsqueeze(torch.as_tensor(num_samples, dtype=torch.int32), 0), 0
    )
    values = [
        (
            padded.numpy(),
            num_of_samples.numpy(),
        )
        for _ in range(40)
    ]

    result0_list = []

    user_data = UserData()

    with grpcclient.InferenceServerClient(
            url=FLAGS.url, verbose=FLAGS.verbose
    ) as triton_client:
        try:
            # Establish stream
            triton_client.start_stream(
                callback=partial(callback, user_data),
                stream_timeout=FLAGS.stream_timeout,
            )
            # Now send the inference sequences...
            async_stream_send(
                triton_client,
                values,
                0,
                FLAGS.model_name,
                FLAGS.model_version,
                FLAGS.topk,
            )
        except InferenceServerException as e:
            print(e)
            sys.exit(1)

        # Retrieve results...
        recv_count = 0
        while recv_count < len(values):
            data_item = user_data._completed_requests.get()
            if type(data_item) == InferenceServerException:
                print(data_item)
                sys.exit(1)
            else:
                result0_list.append(data_item.as_numpy("DECODER_LOGITS"))
            recv_count = recv_count + 1

    print(result0_list)

    print("PASS: Streaming")
