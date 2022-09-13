# image_thing
encode things into images

Currently only supports .png files.

# Brief examples on usage:

Encode a file into an image
image_thing encode "input_image.png" "output_image.png" -f "file_to_encode.txt"

Encode a string into an image 
image_thing encode "input_image.png" "output_image.png" -s "string to encode"

Decode a secret from an image:
image_thing decode "input_image.png" -o "output_file.txt"
(the -o option can be ommitted; if the secret is a string it will oputput to the console,
if it is a file it will be saved to its original filename in the same directory as the input image.)
