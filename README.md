# lawliet

lawliet is a cli tool that plays sound following exit status.

If given command exits with 0, lawliet plays `success.mp3`. If not, it does `failure.mp3`.

`LAWLIET_DIR` must be set and audio files must be there.

## USAGE

```sh
lawliet ls
```

## TODO

- More codec support
- Random play
- Support to add audio files via command line
