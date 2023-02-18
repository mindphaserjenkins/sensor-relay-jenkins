podTemplate(
  label: 'jenkins-slave-rust',
  containers: [
    containerTemplate(
      name: 'jnlp',
      image: 'synology:6000/jenkins-agent-rust-x86',
      alwaysPullImage: true,
      privileged: true
    )
  ]
) {
    node('jenkins-slave-rust') {

        def dockerRegistry = "synology:6000"
        def myRepo = checkout scm
        def version = "1.0." + new Date().format("yyMMddHHmmss", TimeZone.getTimeZone('UTC'))
        def dockerImage = "$dockerRegistry/sensor-relay:$version"

        env.DOCKER_IMAGE = dockerImage

        try {
          stage('Checkout') {
            echo 'Make the output directory'
            sh 'mkdir -p build'

            dir('build') {
              git credentialsId: 'mindphaserjenkins', url: 'git@github.com:mangefoo/sensor-relay.git'
            }
          }

          stage('Build application') {
            echo "Building $dockerImage"
            dir ('build') {
              sh "~/.cargo/bin/cargo build --release"
            }
            echo 'Done'
          }

          stage('Build docker') {
            echo "Building docker image"
            sh "docker build -f Dockerfile.x86 . -t $dockerImage"
            echo "Done building docker image"
          }

          stage('Push docker') {
            echo "Pushing docker image $dockerImage"
            sh "docker push $dockerImage"
            echo "Done pushing docker image"
          }

          stage('Deploy to Kubernetes') {
            echo "Deploying to Kubernetes"
            withKubeConfig([credentialsId: 'microk8s']) {
              sh "envsubst < k8s/sensor-relay.yaml | kubectl apply -n default -f -"
            }
            echo "Done deploying to Kubernetes"
          }

          stage('Send notifications') {
            echo "Sending Slack success notification"
            def userId = slackUserIdFromEmail('magnus@mindphaser.se')
            slackSend(channel: userId, color: "good", message: "Pipeline sensor-relay-jenkins finished!")
            echo "Done sending Slack success notification"
          }
        } catch (all) {

          echo "Sending Slack failure notification"
          def userId = slackUserIdFromEmail('magnus@mindphaser.se')
          slackSend(channel: userId, color: "danger", message: "Pipeline sensor-relay-jenkins failed!")
          echo "Done sending Slack failure notification"
          error("Pipeline failed")
        }
    }
}
