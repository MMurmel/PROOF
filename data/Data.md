# Data

### Purpose
While this project is more about the algorithms than it is about finding value in specific datasets,
we still require a dataset to train with in the first place.

For this purpose a modified selection from
[THE MNIST DATABASE of handwritten digits](http://yann.lecun.com/exdb/mnist/) was chosen.

### Modifications
Since this project is about learning of boolean functions,
features and labels of the data need be of boolean type.
To accomplish this, the subset of handwritten ones and sevens will be used as data,
where a `1` interpreted as labeled `true` and a `7` is interpreted as labeled `false`.

Furthermore, greyscale feature values (`0-255`) need to be converted to boolean.
For this, a simple cut at `127` can be used, s.t. every pixel with brightness `<=127`
will be converted to `false`, while every pixel with brightness `>=128` will be converted to `true`.

We don't need much data for this, especially not the whole training set of 60k images MNist provides.
Therefore, we will only use the test set of 10k images and filter it for ones and sevens.
This results in `1135` images depicting a one and `1028` images depicting a seven.

Most significantly, perhaps, is the change from binary to plaintext data in json format,
simplifying access for the programmer.
The format will be the auto-generated json by the serialization framework [serde](https://github.com/serde-rs/json),
e.g. `{"label":false,"features":[true,true,false]}`.

### Recreating
In order to (re)create the data used for this project
you must create an anaconda environment from the `environment.yaml`
and execute the `convert_data.py` from within the `data` directory.
