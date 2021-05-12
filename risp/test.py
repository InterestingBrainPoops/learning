import re 

s = """
buildscript {
    repositories {
        mavenLocal()
        mavenCentral()
    }
    dependencies {
        classpath 'com.github.docker-java:docker-java:3.1.5'
    }
}

mavenVersioning {
    Project 's'
}

buildscript {
    dependencies {
        classpath 'com.github.docker-java:docker-java:3.1.5'
    }
}
"""

match = re.search(r'buildscript {.*          }.*}', s, re.DOTALL)

text = match.group(0)

print(text)