# 发版

1. 先生成`CHANGELOG.md`

```shell
npx changelogen@latest --release
```

2. 提交`CHANGELOG.md`

```shell
git push origin dev
```

3. 提交`tag`

```shell
git push --tags
```
