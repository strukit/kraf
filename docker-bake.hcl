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

target "_common" {
  context    = "."
  policy     = [{ disabled = true, reset = true }]
}

target "setup" {
  name        = "setup-${platform}-${arch}"
  description = "Builds the environment for one platform/architecture."
  inherits    = ["_common"]
  dockerfile  = "Dockerfile.${platform}"

  matrix = {
    arch     = ["amd64", "arm64"]
    platform = ["windows", "linux"]
  }

  platforms = ["${platform}/${arch}"]

  args = {
    ARCH = arch
    ARCH_RUST = resolve_arch_rust(arch)
    WORKSPACE = resolve_workspace()
  }
}

# docker buildx bake linux-amd64 (linux/amd64)
# docker buildx bake linux-arm64 (linux/arm64)
target "linux" {
  name        = "linux-${arch}"
  dockerfile  = "Dockerfile"
  target      = "builder-artifacts"
  matrix      = { arch = ["amd64", "arm64"] }
  contexts    = { "setup-environment" = "target:setup-linux-${arch}" }
  output      = ["type=local,dest=.dist,platform-split=true"]

  args = {
    WORKSPACE = resolve_workspace()
  }
}

# docker buildx bake windows-amd64 
# docker buildx bake windows-arm64 
target "windows" {
  name        = "windows-${arch}"
  dockerfile  = "Dockerfile"
  target      = "builder-artifacts"
  matrix      = { arch = ["amd64", "arm64"] }
  contexts    = { "setup-environment" = "target:setup-windows-${arch}" }
  output      = ["type=local,dest=.dist,platform-split=true"]

  args = {
    WORKSPACE  = resolve_workspace()
    BINARY_EXT = ".exe"
  }
}

# docker buildx bake test-linux-amd64 
# docker buildx bake test-linux-arm64
# docker buildx bake test-windows-arm64
# docker buildx bake test-windows-amd64 
target "test" {
  name        = "test-${platform}-${arch}"
  dockerfile  = "Dockerfile"
  target      = "tester"
  matrix      = { arch = ["amd64", "arm64"], platform = ["linux", "windows"] }
  contexts    = { "setup-environment" = "target:setup-${platform}-${arch}" }

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
  dockerfile  = "Dockerfile"
  target      = "linter"
  matrix      = { arch = ["amd64", "arm64"], platform = ["linux", "windows"] }
  contexts    = { "setup-environment" = "target:setup-${platform}-${arch}" }

  args = {
    WORKSPACE = resolve_workspace()
  }
}