#Reference Checking

State can be contained by functions or fields in structs. Threrefore everytime a object is passed to a field its reference count must be increased. Also functions getting it as arguments or functions initialising the object increases the reference count. The end of the function delets all localvariables and decreases reference count

[x] reference inc on call
[x] reference in on set complex in field of other complex
[x] NO reference dec if its returnValue
[x] referece deletion on return