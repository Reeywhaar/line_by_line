# line\_by\_line
line\_by\_line is a cli tool written in rust which takes multiple files and outputs them merged line by line

## Usage
```
line_by_line [default_padding=100] [...[path [padding=100]]>1]
```

## Example
```
$ cat test1.txt
The quick brown
fox jumps over
the lazy dog
The quick brown
fox jumps over
the lazy dog

$ cat test2.txt
Lorem ipsum dolor
sit amet, consectetur
adipiscing elit.
Sed pretium varius
est ultricies venenatis.
Phasellus faucibus orci
felis, ut molestie
enim suscipit ac.

$ line_by_line 25 test1.txt test2.txt
The quick brown          Lorem ipsum dolor
fox jumps over           sit amet, consectetur
the lazy dog             adipiscing elit.
The quick brown          Sed pretium varius
fox jumps over           est ultricies venenatis.
the lazy dog             Phasellus faucibus orci
                         felis, ut molestie
                         enim suscipit ac.
