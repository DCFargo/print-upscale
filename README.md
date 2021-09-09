# print-upscale
Upscales images for printing and stitching.

## Examples
WIP
## Usage
For general usage, the following syntax will work: `print-upscale [FLAGS] [OPTIONS] <INPUT> <OUTPUT>`  
  
- In most circumstances, the default provided values will be fine enough. However the `help` flag can clarify specific options.
Also note that the input refers to a specific file, and output refers to a name paired with the specified file type (or pdf if default).  
- If the chosen type is pdf, a single file consisting of all pages will be made output;
however, if an image type is chosen, a folder "processed" will be made containing all of the individual sheets to be printed.
