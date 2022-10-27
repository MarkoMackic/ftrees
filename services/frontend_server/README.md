# Simple proxy server for serving frontend files

Resolves subdomain also ( because we'll use that as specific family and db identifer )

Configuration specifies which Access Rules are required for user to view the page ..

The idea is the following ( supposing app is running on http://example.org )

* example.org -> serves homepage ( a little of introduction to ftrees )
* family.example.org -> serves homepage for specific family ( a view of family tree, with some fine grained access , maybe ?urlToken=...  to protect the page from every viewer )
* example.org/login_page -> authentication/authorization of platform admins ( not related to any family )
* family.example.org/login_page -> same page as the one above ( but defined the database from which users are extracted )
