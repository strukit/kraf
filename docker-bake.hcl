function "resolve_arch_rust" {
  params = [arch]
  result = {
    amd64 = "x86_64"
    arm64 = "aarch64"
  }[arch]
}

function "resolve_workspace" {
  params = []
  result = "/workspace"
}

variable "ARTIFACT_PATH" {
  default = "dist"
}

target "_common" {
  context    = "."
  policy     = [{reset = true }]
  load       = false
}

target "setup" {
  name        = "setup-${platform}-${arch}"
  description = "Builds the environment for one platform/architecture."
  inherits    = ["_common"]
  dockerfile  = "Dockerfile.${platform}"

  matrix = {
    arch     = ["amd64", "arm64"]
    platform = ["linux", "windows"]
  }

  platforms = ["${platform}/${arch}"]

  args = {
    ARCH = arch
    ARCH_RUST = resolve_arch_rust(arch)
    WORKSPACE = resolve_workspace()
  }
}

# docker buildx bake build-linux-amd64 (linux/amd64)
# docker buildx bake build-linux-arm64 (linux/arm64)
target "linux" {
  name        = "build-linux-${arch}"
  inherits    = ["_common"]
  dockerfile  = "Dockerfile"
  target      = "builder-artifacts"
  contexts    = { "setup-environment" = "target:setup-linux-${arch}" }
  output      = ["type=local,dest=${ARTIFACT_PATH},platform-split=true"]

  matrix = {
    arch     = ["amd64", "arm64"]
    platform = ["linux"]
  }

  platforms = ["${platform}/${arch}"]

  args = {
    WORKSPACE = resolve_workspace()
  }
}

# docker buildx bake build-windows-amd64 
# docker buildx bake build-windows-arm64 
target "windows" {
  name        = "build-windows-${arch}"
  inherits    = ["_common"]
  dockerfile  = "Dockerfile"
  target      = "builder-artifacts"
  contexts    = { "setup-environment" = "target:setup-windows-${arch}" }
  output      = ["type=local,dest=dist,platform-split=true"]

  matrix = {
    arch     = ["amd64", "arm64"]
    platform = ["windows"]
  }
  
  platforms = ["${platform}/${arch}"]


  args = {
    WORKSPACE  = resolve_workspace()
    BINARY_EXT = ".exe"
  }
}

# docker buildx bake test-unit-linux-amd64 
# docker buildx bake test-unit-linux-arm64
# docker buildx bake test-unit-windows-arm64
# docker buildx bake test-unit-windows-amd64 
target "test" {
  name        = "test-unit-${platform}-${arch}"
  inherits    = ["_common"]
  dockerfile  = "Dockerfile"
  target      = "tester"
  contexts    = { "setup-environment" = "target:setup-${platform}-${arch}" }

  matrix = {
    arch     = ["amd64", "arm64"]
    platform = ["linux", "windows"]
  }
  
  platforms = ["${platform}/${arch}"]


  args = {
    WORKSPACE = resolve_workspace()
  }
}

# docker buildx bake lint-linux-amd64 
# docker buildx bake lint-linux-arm64
# docker buildx bake lint-windows-arm64
# docker buildx bake lint-windows-amd64 
target "lint" {
  name        = "lint-${platform}-${arch}"
  inherits    = ["_common"]
  dockerfile  = "Dockerfile"
  target      = "linter"
  contexts    = { "setup-environment" = "target:setup-${platform}-${arch}" }


  matrix = {
    arch     = ["amd64", "arm64"]
    platform = ["linux", "windows"]
  }

  platforms = ["${platform}/${arch}"]

  args = {
    WORKSPACE = resolve_workspace()
  }
}

group "linux-amd64" {
  targets = [ "build-linux-amd64", "lint-linux-amd64", "test-unit-linux-amd64" ]
}

group "linux-arm64" {
  targets = [ "build-linux-arm64", "lint-linux-arm64", "test-unit-linux-arm64" ]
}