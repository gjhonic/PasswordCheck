let app = new Vue({
    el: '#app',
    data: {
        config: {
            projectName: 'Password Check',
            version: '0.1',
            lang: 'en',
            author: 'gjhonic',
        },

        actionCheckPassword: 'check_scam_password',
        password: '',
        message: '',
        statusCode: null,
        status: 0,
        level: 0,
        
        statusErrorValidate: 1,
        statusWeekPassword: 2,
        statusGoodPassword: 3,
        
    },
    computed: {
        
    },
    methods: {
        /**
         *  Метод отправляет запрос на проверку пароля
         */
        async checkPass() {
            let url = 'api/v1/' + this.actionCheckPassword;
            
            let headers = new Headers();
            headers.append("Content-Type", "application/x-www-form-urlencoded");


            let urlencoded = new URLSearchParams();
            urlencoded.append("password", this.password);

            var requestOptions = {
              method: 'POST',
              headers: headers,
              body: urlencoded,
            };

            try {
                let response = await fetch(url, requestOptions);
                let data = await response.json();
                if (response.ok) {
                    if (data !== []) {
                        this.status = data.status;
                        this.message = data.message;
                        this.level = data.level;
                    }
                } else {
                    console.log('Ошибка')
                }
            } catch (e) {
                console.log('Ошибка. ' + e);
            }
        },
    },
});