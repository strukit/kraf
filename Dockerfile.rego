package docker

default allow := false

allow if input.local
allow if input.git

allow if {
  input.image.host == "docker.io"  # Docker Hub
}

allow if {
  input.image.host == "ghcr.io"  # GitHub Container Registry
}

allow if {
  input.image.host == "dhi.io"  # Docker Hardened Images
}

allow if {
  input.image.host == "registry.access.redhat.com"  # Docker Hardened Images
}

allow if {
  input.http.schema == "https"
}

decision := {"allow": allow}