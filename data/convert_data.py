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


def prepare_data(image_file, label_file, out_file, line_count):
    with gzip.open(image_file, "rb") as images, gzip.open(label_file, "rb") as labels, open(out_file, "w") as output_file:
        images.read(16)
        labels.read(8)
        output = []

        for i in range(line_count):
            label = ord(labels.read(1))
            if label != 1 and label != 7:
                continue
            label = (label == 1)
            features = []
            sample = Sample(label=label, features=features)
            for j in range(28*28):
                pixel = ord(images.read(1))
                pixel = (pixel >= 128)
                sample.features.append(pixel)
            output.append(sample)

        for s in output:
            output_file.write(to_json(s) + "\n")


prepare_data("original/t10k-images-idx3-ubyte.gz",
             "original/t10k-labels-idx1-ubyte.gz",
             "output.json",
             10000)
