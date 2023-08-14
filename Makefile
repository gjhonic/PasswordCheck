C_RED        = \033[0;31m
CE           = \033[0m
C_GREEN      = \033[0;32m
C_YELLOW     = \033[0;33m

#### Helper Functions
# Pretty print for `make help`
HELP_FUNCTION =                                                              \
    %help; while(<>){push@{$$help{$$2//'Misc'}},[$$1,$$3]                    \
    if/^([\w-_]+)\s*:.*\#\#(?:@(\w+))?\s(.*)$$/};                            \
    print"$$_:\n", map"  $$_->[0]".(" "x(30-length($$_->[0])))."$$_->[1]\n", \
    @{$$help{$$_}},"\n" for keys %help;

# Render colored title before executing a command
define title
    @echo ""
    @echo -e "$(C_YELLOW)>>>> >>>> >>>> >>>> >>>> >>>> $(C_TITLE) $(1) $(CE)"
endef

help: ## Список команд
	@echo "$(C_YELLOW)                                                    .___        .__                   __     $(CE)"
	@echo "$(C_YELLOW)___________    ______ ________  _  _____________  __| _/   ____ |  |__   ____   ____ |  | __ $(CE)"
	@echo "$(C_YELLOW)\____ \__  \  /  ___//  ___/\ \/ \/ /  _ \_  __ \/ __ |  _/ ___\|  |  \_/ __ \_/ ___\|  |/ / $(CE)"
	@echo "$(C_YELLOW)|  |_> > __ \_\___ \ \___ \  \     (  <_> )  | \/ /_/ |  \  \___|   Y  \  ___/\  \___|    <  $(CE)"
	@echo "$(C_YELLOW)|   __(____  /____  >____  >  \/\_/ \____/|__|  \____ |   \___  >___|  /\___  >\___  >__|\_\ $(CE)"
	@echo "$(C_YELLOW)|__|       \/     \/     \/                          \/       \/     \/     \/     \/        $(CE)"
	@echo "$(C_GREEN)                                                                        password_check        $(CE)"
	@echo ""
	@perl -e '$(HELP_FUNCTION)' $(MAKEFILE_LIST)
	@echo ""


#### Projects Actions ###################################################################################################
build: ##@Projects Запуск раст приложения на локале
	@echo "$(C_GREEN)Build project $(CE)"
	cd backend; cargo run -- 1

up: ##@Projects Запуск проекта в режиме терминал
	@echo "$(C_GREEN)Build docker project $(CE)"
	sudo docker-compose build

	@echo "$(C_GREEN)Up docker project $(CE)"
	sudo docker-compose up


upd: ##@Projects Запуск проекта в режиме демона
	@echo "$(C_GREEN)Build docker project $(CE)"
	sudo docker-compose build

	@echo "$(C_GREEN)Up docker project in demon$(CE)"
	sudo docker-compose up -d


stop: ##@Projects Останавливает проект
	@echo "$(C_GREEN)Stop rust project $(CE)"
	sudo docker stop password_check_backend_1
	@echo ""

	@echo "$(C_GREEN)Stop nginx proxy$(CE)"
	sudo docker stop password_check_proxy_1
	@echo ""

	@echo "$(C_GREEN)Project stopped$(CE)"


del: ##@Projects Удаляет контейнеры
	@echo "$(C_GREEN)Stop rust project $(CE)"
	sudo docker stop passwordcheck_backend_1
	@echo "$(C_RED)Delete rust project $(CE)"
	sudo docker rm passwordcheck_backend_1
	@echo ""

	@echo "$(C_GREEN)Stop nginx proxy$(CE)"
	sudo docker stop passwordcheck_proxy_1
	@echo "$(C_RED)Delete nginx proxy $(CE)"
	sudo docker rm passwordcheck_proxy_1
	@echo ""

	@echo "$(C_GREEN)Dockers containers deleted$(CE)"

del-proxy: ##@Projects Удаляет прокси контейнер (что бы статиску обновить)
	@echo "$(C_GREEN)Stop nginx proxy$(CE)"
	sudo docker stop passwordcheck_proxy_1
	@echo "$(C_RED)Delete nginx proxy $(CE)"
	sudo docker rm passwordcheck_proxy_1
	@echo ""

	@echo "$(C_GREEN)Dockers container deleted$(CE)"	