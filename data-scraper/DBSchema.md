# TABLES

## Supermarket price history
 - id - int
 - timestamp - DateTime
 - supermarket - ForeignKey
 - price - float
 - onSpecial - float
 - originalPrice - float

## Supermarkets being scraped
 - Supermarket ID
 - Supermarket Brand
 - Supermarket Location
 - Supermarket Slug

## Product DB
 - productTitle - string
 - productVariety - string
 - productBrand - string
 - barcode - string
 - size - number
 - unit - string
 - quantity - number