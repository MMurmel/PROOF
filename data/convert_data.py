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


def prepare_data(image_file, label_file, out_file, sample_count, out_file_short, sample_count_short):
    with gzip.open(image_file, "rb") as images, gzip.open(label_file, "rb") as labels, open(out_file, "w") as output_file, open(out_file_short, "w") as output_file_short:
        # Skip header data
        images.read(16)
        labels.read(8)

        output = []

        for i in range(10000):
            # read label
            label = ord(labels.read(1))
            # labels other than 1 and 7 can be ignored
            if label != 1 and label != 7:
                # skip bytes in image file accordingly
                images.read(28 * 28)
                continue
            # convert label to boolean
            label = (label == 1)
            # read features
            features = []
            for j in range(28 * 28):
                pixel = ord(images.read(1))
                pixel = (pixel >= 128)
                features.append(pixel)
            # create sample and add to output
            sample = Sample(label=label, features=features)
            output.append(sample)

        output_file.write(to_json(output[0:sample_count]))
        output_file_short.write(to_json(output[0:sample_count_short]))


prepare_data("original/t10k-images-idx3-ubyte.gz",
             "original/t10k-labels-idx1-ubyte.gz",
             "prepared_data.json",
             30,
             "prepared_data_short.json",
             10)
