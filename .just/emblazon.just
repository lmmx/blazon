# Update the blazon-generated badges in README.md using textum
update-readme-badges:
    just reinstall
    blazon -b blazon -c blazon
