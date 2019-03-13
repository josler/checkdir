Gets the checksum of a directory by firstly computing the checksum of all files within it (recursively), and then using that list to create an overall checksum. It does not take into account file ownership, permissions, or timestamps - just the content.

Algorithm is:

* Get each file
* Sort by path
* For each file, read and calculate md5
* Prepare overall file to calculate with format `$md5 $relative_filename\n`
* Md5 this for the result

It's pretty fast, and largely CPU bound:

![image](https://user-images.githubusercontent.com/167061/54297382-2251c400-458d-11e9-81c6-17c1df7a074f.png)
