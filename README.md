Gets the checksum of a directory by firstly computing the checksum of all files within it (recursively), and then using that list to create an overall checksum. It does not take into account ownership permissions etc.

Algorithm is:

* Get each file
* Sort by path
* For each file, read and calculate md5
* Prepare overall file to calculate with format `$md5 $relative_filename\n`
* Md5 this for the result