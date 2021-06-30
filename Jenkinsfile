podTemplate(
  label: 'jenkins-slave-rust-5',
  nodeSelector: 'memory: hi',
  containers: [
    containerTemplate(
      name: 'jnlp',
      image: 'synology:6000/inbound-agent-rust-docker:1.5',
      resourceRequestMemory: '1200Mi',
      resourceLimitMemory: '2000Mi',
      alwaysPullImage: true,
      privileged: true
    )
  ]
) {
    node('jenkins-slave-rust-5') {

        def dockerRegistry = "synology.int.mindphaser.se:6000"
        def myRepo = checkout scm
        def version = "1.0." + new Date().format("yyMMddHHmmss", TimeZone.getTimeZone('UTC'))
        def dockerImage = "$dockerRegistry/sensor-relay:$version"

        env.DOCKER_IMAGE = dockerImage

        stage('Build application') {
          echo "Building $dockerImage"
          sh "cargo build"
          echo 'Done'
        }

        stage('Build docker') {
          echo "Building docker image"
          sh "docker build . -t $dockerImage"
          echo "Done building docker image"
        }

        stage('Push docker') {
          echo "Pushing docker image $dockerImage"
          sh "docker push $dockerImage"
          echo "Done pushing docker image"
        }

        stage('Deploy to Kubernetes') {
          echo "Deploying to Kubernetes"
          kubernetesDeploy(kubeconfigId: 'Pi-Cluster',
                           configs: 'k8s/sensor-relay.yaml',
                           enableConfigSubstitution: true,
          )
          echo "Done deploying to Kubernetes"
        }

        stage('Send notifications') {
          echo "Sending Slack notification"
          def userId = slackUserIdFromEmail('magnus@mindphaser.se')
          slackSend(channel: userId, color: "good", message: "Pipeline VideoIndexer finished!")
          echo "Done sending Slack notification"
        }
    }
}
