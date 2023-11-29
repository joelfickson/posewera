'use strict';
import  Hapi from '@hapi/hapi';

const init = async () => {

    const server = Hapi.server({
        port: 3000,
        host: 'localhost'
    });

    server.route({
        method: 'GET',
        path: '/',
        handler: (_request, _res) => {

            const simpleResponse = {
                message: 'Hello World!',
                error: false
            };

            return simpleResponse


        }
    });

    await server.start();
    console.log('Server running on %s', server.info.uri);
};


init();