let app = new Vue({
    el: '#app',
    data: {
        config: {
            projectName: 'Password Check',
            version: '0.1',
            lang: 'en',
            author: 'gjhonic',
        },

        actionCheckPassword: 'check_password',
        password: '123',
        message: '',
        statusCode: null,
        status: 0,
        
    },
    computed: {
        
    },
    methods: {
        /**
         *  Метод отправляет запрос на проверку пароля
         */
        async checkPass() {
            let url = 'http://127.0.0.1:3000/api/v1/' + this.actionCheckPassword;
            
            let headers = new Headers();
            headers.append("Content-Type", "application/x-www-form-urlencoded");

            let urlencoded = new URLSearchParams();
            urlencoded.append("password", this.password);

            var requestOptions = {
              method: 'POST',
              headers: headers,
              body: urlencoded,
              redirect: 'follow'
            };

            try {
                let response = await fetch(url, requestOptions);
                let data = await response.json();
                if (response.ok) {
                    if (data !== []) {
                        this.statusCode = data.status;
                        this.message = data.message;
                        this.setStatus();
                    }
                } else {
                    console.log('Ошибка')
                }
            } catch (e) {
                console.log('Ошибка. ' + e);
            }
        
        },
        /**
         *  Метод устанавливает статус
         */
        setStatus() {
            if (this.statusCode === false) {
                this.status = 1;
            } else if (this.statusCode === true) {
                this.status = 2;
            }
        }
    },
});