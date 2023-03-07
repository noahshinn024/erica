# Erica
Erica can see everything on your screen. Ask her to help you with something.

### WARNING
There is a reason why Erica is very helpful. She can see everything on your screen. Don't send her messages if you are not prepared for her to take screenshots.

## Requirements
`rust`

On Linux, you need `libxcb`, `libxrandr`, and `dbus`

You also need [Tesseract](https://tesseract-ocr.github.io/tessdoc/Installation.html) for image and [Leptonica](http://www.leptonica.org/) for image processing.

## Installation

Clone this repo:
```
git clone https://github.com/noahshinn024/erica
```

Run `make`:
```
cd ./erica && make
```

## To Run

Add an OpenAI API key:
```bash
export OPENAI_API_KEY="<your api key>"
```

```
erica <a message to Erica>
```

## Examples

```
>>> erica "hey erica, what am I doing right now?"
```

<p>
Based on the content on your screen, it seems like you are viewing a webpage or a text document that contains information about programming in Rust.
</p>

```
>>> erica "Can you summarize the article that I'm reading?"
```
<p>
  It appears that you are reading a programming article about error handling in Rust. The article discusses when to use the `panic!` function and when to return a `Result` value. It suggests that `panic!` should be used in situations where there is no way to recover from an error, while `Result` should be used when the calling code needs options to recover from an error. The article also provides guidelines on when to use `panic!` in library code and when to use it in examples, prototype code, and tests. Finally, the article discusses how to write automated tests and how to handle errors in them.
</p>

```
>>> erica "Finish the code for the forward pass of the Disciminator that I'm writing."
```
<p>
Sure! Here's the completed code for the forward pass of the Discriminator:
</p>
```python
class Discriminator(nn.Module):
    def __init__(self, in_image):
        super(Discriminator, self).__init__()
        self.fc1 = nn.Linear(in_image, 128)
        self.fc2 = nn.Linear(128, 1)
        self.relu = nn.LeakyReLU(0.01)
        self.sigmoid = nn.Sigmoid()

    def forward(self, x):
        x = self.fc1(x)
        x = self.relu(x)
        x = self.fc2(x)
        x = self.sigmoid(x)
        return x
```
<p>
Note that I changed `self.11` and `self.12` to `self.fc1` and `self.fc2` respectively, as it is more common to use this naming convention for fully connected layers in PyTorch.
</p>

## Under Development
  - multi-screen/monitor image processing
  - `lite-mode` for faster messaging with low quality images
  - common application recognition for better responses
