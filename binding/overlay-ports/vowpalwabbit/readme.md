If anything in this directory is modified then the cache must be deleted and regenerated.

To delete the cache you can use the `gh` tool (or REST):
```
gh api https://api.github.com/repos/jackgerrits/vowpalwabbit-rs/actions/caches
```

Then for each cache call this with the `CACHE_ID`:
```
gh api -X DELETE https://api.github.com/repos/jackgerrits/vowpalwabbit-rs/actions/caches/CACHE_ID
```
