pipeline {
    agent any
    stages {
        dir('ocarina-os') {
            stage('Docker Container Work') {
                agent {
                    dockerfile true 
                }
                stages {
                    stage('Hello World!') {
                        steps {
                            echo 'Hello world!'
                        }
                    }
                }
            }
        }
    }
}