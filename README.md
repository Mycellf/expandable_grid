This crate provides a simple, expandable 2d grid which can be accessed from arbitrary signed isize coordinates. It stores its data in a unified area in memory, and copies it to another allocation when changing size like a vec does. It always takes up the minimum amount of space for its accessible size, however, it also overallocates when resizing to fit an area to reduce allocations. 
