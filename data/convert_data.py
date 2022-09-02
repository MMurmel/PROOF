#!/usr/bin/env python
import gzip
from dataclasses import dataclass
from typing import List

from serde import serialize, deserialize
from serde.json import to_json


@serialize
@deserialize
@dataclass
class Sample:
    label: bool
    features: List[bool]


def prepare_data(image_file, label_file, out_file, sample_count):
    with gzip.open(image_file, "rb") as images, gzip.open(label_file, "rb") as labels, open(out_file, "w") as output_file:
        # Skip header data
        images.read(16)
        labels.read(8)

        output = []

        for i in range(sample_count):
            # read label
            label = ord(labels.read(1))
            # labels other than 1 and 7 can be ignored
            if label != 1 and label != 7:
                # skip bytes in image file accordingly
                images.read(28*28)
                continue
            # convert label to boolean
            label = (label == 1)
            # read features
            features = []
            for j in range(28*28):
                pixel = ord(images.read(1))
                pixel = (pixel >= 128)
                features.append(pixel)
            # create sample and add to output
            sample = Sample(label=label, features=features)
            output.append(sample)

        for s in output:
            output_file.write(to_json(s) + "\n")
        # TODO
        #output_file.write(to_json(output))


prepare_data("original/t10k-images-idx3-ubyte.gz",
             "original/t10k-labels-idx1-ubyte.gz",
             "prepared_data.json",
             10000)

with open("prepared_data.json") as whole_file, open("prepared_data_short.json", "w") as short_file:
    for _ in range(10):
        line = next(whole_file).strip()
        short_file.write(line + "\n")
