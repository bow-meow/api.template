# Configuration Information

This isn't currently configurable, so here's some info:
- **Listens on host:port**: localhost:81
- **Uses database**: devserver:schema24-testing

## Endpoints:

### Stills:

- **GET by Entity**:  
  `http://<server>:81/stills/entity?entityId=<entityid>&entityType=<entitytype>`
  
  - **Params**:
    - `entityId`: The ID of the entity the stillimage belongs to.
    - `entityType`: Entity type can be: `product`, `productgroup`, or `customview`.

  - Example:  
  `http://localhost:81/stills/entity?entityId=1369&entityType=product`

- **GET by ID**:  
  `http://<server>:81/stills/<id>`
  
  - **Params**:
    - `id`: The ID of the still image.

  - Example:  
  `http://localhost:81/stills/5`
