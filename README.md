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

```echo '{"a":"{\"a1\":1}","b":["{\"w\":2}"]}' | jsu```
Outputs:
```
{
  "a": "{\"a1\":1}",
  "b": [
    "{\"w\":2}"
  ]
}
```

adding the -e flag will expand the embeded strings:

```echo '{"a":"{\"a1\":1}","b":["{\"w\":2}"]}' | jsu -e ```

Outputs:
```
{
  "a": {
    "a1": 1
  },
  "b": [
    {
      "w": 2
    }
  ]
}
```

## To extract json segments from a larger text file

'''echo 'some text {"a":1} more "text" [1,2,3] text [1,{"b":2}] text ' | jsu -x```

Outputs:
```
[
  {
    "a": 1
  },
  [
    1,
    {
      "b": 2
    }
  ]
]
```
