# Uniqopy

Create a unique copy of a file using its name, the system time, and
[MD5 hashing](https://en.wikipedia.org/wiki/MD5).

```shell
$ ls
fibblesnork.txt
$ uniqopy fibblesnork.txt 
Copying fibblesnork.txt to fibblesnork.2022-01-10-07:06:49.db194cb65e3d5200798471729c8f3e9a.txt
Copyied 16 bytes
$ ls
fibblesnork.2022-01-10-07:06:49.db194cb65e3d5200798471729c8f3e9a.txt
fibblesnork.txt
```

You might want a unique filename:

* For files generated in a batch job
* For (small) backups
* For log rotation
* Etc.


# Name Generation

To generate a uniquely named copy, uniqopy:

1. reads and calculates the MD5 hash of the input file's contents (with
   [`std::fs::read`](https://doc.rust-lang.org/std/fs/fn.read.html) and the
   [`md5`](https://docs.rs/md5/0.7.0/md5/) crate),
1. generates a timestamp (with
   [`chrono::offset::Local::now`](https://docs.rs/chrono/0.4.19/chrono/offset/struct.Local.html#method.now)),
1. concatenates the file's original name, the timestamp, and the MD5 hash, and
   finally
1. if the original filename has an extension (according to
   [`std::path::Path::extension`](https://doc.rust-lang.org/std/path/struct.Path.html#method.extension)) it's moved to the end of the new filename.

For example:
* `foo` becomes 
`foo.2022-01-10-07:05:03.d3b07384d113edec49eaa6238ad5ff00`
* `foo.txt` becomes `foo.2022-01-10-07:05:01.0e771a9094f21f0bf74be99ebdbb568d.txt`


# License

This work is provided under the **Parity Public License 7.0.0** which can be
found in [`LICENSE.md`](./LICENSE.md) or at
https://paritylicense.com/versions/7.0.0.
