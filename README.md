#Seam carving
This project calculates the seam with the lowest energy. 
A seam is like a path of pixels. 
The lowest energy means that the seam does not contain a lot of information and can 
be safely removed. If you do this iteratively you are resizing the image in a content-aware way.
The idea is illustrated in the following example.
![Original](images/surfer.jpg?raw=true "original")
![Resized](images/output.jpg?raw=true "resized")

#Running the project
Clone the project and run
```
cargo install --path ./
```
This will install it in to the /.cargo/bin folder. If you export this path to the terminal. You will be able to ru n

```
contenet-aware-resizing -i <input-path-image> -o <output-path-image> -n <number-of-times-you-want-to-crop>
```