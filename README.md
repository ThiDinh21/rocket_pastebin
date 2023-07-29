# rocket_pastebin

After using ```cargo run```, use ```curl``` to upload and retrieve data:

- upload: ```curl --data-binary @test.txt https://localhost:8000/```. The program will return an URL to the text file, e.g. "https://localhost:8000/xfH"
- retrieve: ```curl https://localhost:8000/xfH``` will return the content of ```test.txt```
