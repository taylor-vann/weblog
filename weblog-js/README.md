# Weblog-js

The javascript side of taylorvann dot com.

Typescript js monorepo

Build TS into JS and copy to a directory like `www/scripts/`

`npm run build_deps`
-> creates all the bundled stuff needed
--> puts it in www/scripts/libs
`npm run build_site`
-> directly lives in www/
-> directly correlates to page structure because it is up
	in that page structure

Requirements:
There will be certain libraries required by many pages.
These should be linked through "import maps".
This also keeps versioning inline.

1) bundle required libraries into a `/libs` directory
	- so `wctk` becomes `/wctk_0.js`
2) then there's a "web app" side of js
	- libraries are imported by import maps


