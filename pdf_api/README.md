# PDF_API
This api will extract text from PDF, which will make them searchable.

* Tesseract: Is an open-source optical character recognition (OCR) engine developed by Google. It is widely used for extracting text from images, scanned documents, and PDF files.
* Pdfium: Is an open-source PDF rendering engine developed by Google. It allows developers to work with PDF files programmatically.
* export LD_LIBRARY_PATH=./pdf_api/pdfium/{platform}/lib
  * change {platform} to windows/linux/osx
* Linux:
  * sudo apt-get install -y tesseract-ocr
  * sudo apt-get install -y libtesseract-dev
* Windows:
  * choco install tesseract
* OSX
  * brew install tesseract

## Usage

* `cd pdf_api` and `curl -X POST -F "file=@file.pdf" http://localhost:3017/upload`