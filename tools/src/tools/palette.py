# From a input image representing a color palette for a given hardware,
# output the list of RGB values as hexadecimal values.
# The tool will read the image row-by-row.

import argparse

import imageio.v3 as iio
import numpy as np


def main() -> None:
    parser = argparse.ArgumentParser(
        prog="Palette processor",
        description="Read a palette file and extract the RGB hex values",
    )
    parser.add_argument("-i", "--input")
    parser.add_argument("-o", "--output")
    args = parser.parse_args()

    # Open the input image requested
    image = iio.imread(args.input)
    dim = image.shape[:2]

    # Prepare the target output file to write the data
    with open(args.output, "+wt") as output:
        for index, (x, y) in enumerate(np.ndindex(dim)):
            # Get the RGB channels of the pixel
            pixel = image[x, y]
            r = int(pixel[0])
            g = int(pixel[1])
            b = int(pixel[2])

            # Prepare the text line to write
            line = f"([0x{r:0>2x}, 0x{g:0>2x}, 0x{b:0>2x}], 0x{index:0>2x}),\n"
            output.write(line)
