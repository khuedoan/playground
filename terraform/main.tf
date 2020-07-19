variable "project_id" {
  default = "kubernetes-lab-283716"
}

provider "google" {
  project = var.project_id
  region  = "asia-southeast2"
  zone    = "asia-southeast2-a"
}

# Enable Compute Engine API
resource "google_project_service" "compute_engine_api" {
  service                    = "compute.googleapis.com"
  disable_dependent_services = true
}

# gcloud compute networks create kubernetes-the-hard-way --subnet-mode custom
resource "google_compute_network" "kubernetes_vpc_network" {
  name                    = "kubernetes-the-hard-way"
  auto_create_subnetworks = false
  depends_on = [
    google_project_service.compute_engine_api
  ]
}

# gcloud compute networks subnets create kubernetes \
#   --network kubernetes-the-hard-way \
#   --range 10.240.0.0/24
resource "google_compute_subnetwork" "kubernetes_subnetwork" {
  name          = "kubernetes"
  network       = google_compute_network.kubernetes_vpc_network.id
  ip_cidr_range = "10.240.0.0/24"
}

# gcloud compute firewall-rules create kubernetes-the-hard-way-allow-internal \
#   --allow tcp,udp,icmp \
#   --network kubernetes-the-hard-way \
#   --source-ranges 10.240.0.0/24,10.200.0.0/16
resource "google_compute_firewall" "kubernetes_firewall_internal" {
  name    = "kubernetes-the-hard-way-allow-internal"
  network = google_compute_network.kubernetes_vpc_network.id

  allow {
    protocol = "tcp"
  }

  allow {
    protocol = "udp"
  }

  allow {
    protocol = "icmp"
  }

  source_ranges = [
    "10.240.0.0/24",
    "10.200.0.0/16"
  ]
}

# gcloud compute firewall-rules create kubernetes-the-hard-way-allow-external \
#   --allow tcp:22,tcp:6443,icmp \
#   --network kubernetes-the-hard-way \
#   --source-ranges 0.0.0.0/0
resource "google_compute_firewall" "kubernetes_firewall_external" {
  name    = "kubernetes-the-hard-way-allow-external"
  network = google_compute_network.kubernetes_vpc_network.id

  allow {
    protocol = "tcp"
    ports = [
      "22",
      "6443"
    ]
  }

  allow {
    protocol = "icmp"
  }

  source_ranges = [
    "0.0.0.0/0"
  ]
}

# gcloud compute addresses create kubernetes-the-hard-way \
#   --region $(gcloud config get-value compute/region)
resource "google_compute_global_address" "default" {
  name = "kubernetes-the-hard-way"
}

# for i in 0 1 2; do
#   gcloud compute instances create controller-${i} \
#     --async \
#     --boot-disk-size 200GB \
#     --can-ip-forward \
#     --image-family ubuntu-2004-lts \
#     --image-project ubuntu-os-cloud \
#     --machine-type e2-standard-2 \
#     --private-network-ip 10.240.0.1${i} \
#     --scopes compute-rw,storage-ro,service-management,service-control,logging-write,monitoring \
#     --subnet kubernetes \
#     --tags kubernetes-the-hard-way,controller
# done
resource "google_compute_instance" "kubernetes_controllers" {
  count          = 3
  name           = "controller-${count.index}"
  machine_type   = "e2-standard-2"
  can_ip_forward = true

  boot_disk {
    initialize_params {
      size = 200
      image = "ubuntu-os-cloud/ubuntu-2004-lts"
    }
  }

  network_interface {
    subnetwork = google_compute_subnetwork.kubernetes_subnetwork.id
    network_ip = "10.240.0.1${count.index}"
  }

  service_account {
    scopes = [
      "compute-rw",
      "storage-ro",
      "service-management",
      "service-control",
      "logging-write",
      "monitoring"
    ]
  }

  tags = [
    "kubernetes-the-hard-way",
    "controller"
  ]
}

# for i in 0 1 2; do
#   gcloud compute instances create worker-${i} \
#     --async \
#     --boot-disk-size 200GB \
#     --can-ip-forward \
#     --image-family ubuntu-2004-lts \
#     --image-project ubuntu-os-cloud \
#     --machine-type e2-standard-2 \
#     --metadata pod-cidr=10.200.${i}.0/24 \
#     --private-network-ip 10.240.0.2${i} \
#     --scopes compute-rw,storage-ro,service-management,service-control,logging-write,monitoring \
#     --subnet kubernetes \
#     --tags kubernetes-the-hard-way,worker
# done
resource "google_compute_instance" "kubernetes_workers" {
  count          = 3
  name           = "worker-${count.index}"
  machine_type   = "e2-standard-2"
  can_ip_forward = true

  boot_disk {
    initialize_params {
      size = 200
      image = "ubuntu-os-cloud/ubuntu-2004-lts"
    }
  }

  network_interface {
    subnetwork = google_compute_subnetwork.kubernetes_subnetwork.id
    network_ip = "10.240.0.2${count.index}"
  }

  service_account {
    scopes = [
      "compute-rw",
      "storage-ro",
      "service-management",
      "service-control",
      "logging-write",
      "monitoring"
    ]
  }

  metadata = {
    pod-cidr = "10.200.${count.index}.0/24"
  }

  tags = [
    "kubernetes-the-hard-way",
    "worker"
  ]
}
