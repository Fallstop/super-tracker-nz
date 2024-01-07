# TABLES

## Supermarket price history
 - id - int
 - timestamp - DateTime
 - productID - ForeignKey
 - supermarket - ForeignKey
 - price - float
 - onSpecial - bool
 - originalPrice - float

## Supermarkets being scraped
 - Supermarket ID
 - Supermarket Name
 - Supermarket Brand
 - Supermarket Location
 - Supermarket LocationID

## Product DB
 - productID
 - productTitle - string
 - productVariety - string
 - productBrand - string
 - barcode - string
 - size - number
 - unit - string
 - quantity - number
 - imageURL - string