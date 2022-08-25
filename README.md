# jsu
JSon Utils


## This can be used to pretty print json
By default it will output a nicely formated json output 

```echo '{"a":1,"b":2,"c":[1,2,3]}' | jsu```

Output :
```
{
  "a": 1,
  "b": 2,
  "c": [
    1,
    2,
    3
  ]
}
```

## This can be used to compact print json
```echo '{ "a":1, "b":2, "c":[1,2,3] }' | jsu -c```

Output:
`{"a":1,"b":2,"c":[1,2,3]}`


## To expand any json embedded strings in json

## To extract json segments from a larger text file
