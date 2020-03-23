# Are we mean today?
[![Codeship Status for LesnyRumcajs/arewemeantoday](https://app.codeship.com/projects/5e581c40-4f7a-0138-b9da-2213627debf2/status?branch=master)](https://app.codeship.com/projects/389950)

Daily grumpiness compass. 
https://arewemeantoday.herokuapp.com/


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
