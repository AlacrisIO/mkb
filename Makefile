# make VERBOSE nonempty to see raw commands (or provide on command line)
ifndef VERBOSE
VERBOSE:=
endif

# use SHOW to inform user of commands
SHOW:=@echo

# use HIDE to run commands invisibly, unless VERBOSE defined
HIDE:=$(if $(VERBOSE),,@)

# Our applications recognize this as the top directory for the project, and look for files there
# at runtime, e.g. for configuration.
export ALACRIS_HOME:=$(shell pwd)

.PHONY: docker-build-all docker-build docker-list docker-up docker-start \
        docker-stop docker-restart docker-status docker-clean docker-prune

# Docker part
DOCKER_COMPOSE = docker-compose
DOCKER_COMPOSE_FILE = docker/docker-compose.yml

docker-pull: ## Pull Alacris prerequisites images
	$(SHOW) " Pulling Alacris Docker images"
	$(HIDE) docker/scripts/pull_images.sh

## Build all containers in foreground (including build-prerequisites)
docker-build-all:
	$(SHOW) " Building Alacris MKB Docker images"
	$(HIDE) docker/scripts/build_all_images.sh

docker-build: ## Build all or c=<name> containers in foreground
	@$(DOCKER_COMPOSE) -f $(DOCKER_COMPOSE_FILE) build $(c)

docker-list: ## List available services
	@$(DOCKER_COMPOSE) -f $(DOCKER_COMPOSE_FILE) config --services

docker-up: ## Start all or c=<name> containers in foreground
	@$(DOCKER_COMPOSE) -f $(DOCKER_COMPOSE_FILE) up $(c)

docker-start: ## Start all or c=<name> containers in background
	@$(DOCKER_COMPOSE) -f $(DOCKER_COMPOSE_FILE) up -d $(c)

docker-stop: ## Stop all or c=<name> containers
	@$(DOCKER_COMPOSE) -f $(DOCKER_COMPOSE_FILE) stop $(c)

docker-restart: ## Restart all or c=<name> containers
	@$(DOCKER_COMPOSE) -f $(DOCKER_COMPOSE_FILE) stop $(c)
	@$(DOCKER_COMPOSE) -f $(DOCKER_COMPOSE_FILE) up -d $(c)

docker-status: ## Show status of containers
	@$(DOCKER_COMPOSE) -f $(DOCKER_COMPOSE_FILE) ps

docker-logs: ## Show logs for all or c=<name> containers
	@$(DOCKER_COMPOSE) -f $(DOCKER_COMPOSE_FILE) logs --tail=100 -f $(c)

docker-clean:  ## Clean all data
	@$(DOCKER_COMPOSE) -f $(DOCKER_COMPOSE_FILE) down -v

docker-prune: ## Delete dangling images
	$(SHOW) Deleting dangling docker images
	$(HIDE) docker system prune