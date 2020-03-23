# Are we mean today?
Daily grumpiness compass. 


---
### Buidling & deploying Rocket-Heroku application
Ensure you have `heroku-cli` installed on your machine.
```
heroku create --buildpack emk/rust arewemeantoday
echo 'web: ROCKET_PORT=$PORT ROCKET_ENV=prod ./target/release/arewemeantoday' > Procfile
# Rocket requires nightly Rust at the moment.
echo 'VERSION=nightly' > RustConfig

... # do the development, commit what you have

# if first run
git push --set-upstream heroku master
# consecutive ones
git push heroku master
```

That's really all! Enjoy! 🦀🚀
