## Rocketseed interview task 

- create /transform post endpoint 
- transform html text to eiher uppercase or lowercase.

```
Input:
{
    "transform": "uppercase",
    "html: "<p>Hello world</p>"
}
Output:
{
    "<p>HELLO WORLD</p>"
}
```

```bash
curl -X POST http://localhost:8080/transform -d '{"transform": "uppercase", "html": "<p>Hello world</p>"}' -H "Content-Type: application/json"
```