# -*- mode: ruby -*-
# vi: set ft=ruby :

Vagrant.configure("2") do |config|
  config.vm.box = "precise64"
  config.vm.box_url = "http://files.vagrantup.com/precise64.box"
  config.vm.network :private_network, ip: "192.168.0.42"
  config.vm.synced_folder "~/code/rust", "/vagrant", :nfs => true
  config.vm.provision :shell do |shell|
    shell.inline = <<SCRIPT
      set -x
      set -e
      # apt-get update
      # apt-get install -y build-essential git
      sudo -iu vagrant bash -exc "
        git clone /vagrant rust;
        cd rust;
        git pull;
        ./configure;
        make;
      "
SCRIPT
  end
end
