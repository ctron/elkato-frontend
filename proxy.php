<?php

use Monolog\Handler\StreamHandler;
use Monolog\Logger;
use Symfony\Component\HttpClient\HttpClient;

require __DIR__.'/vendor/autoload.php';

ini_set('display_errors', '1');
ini_set('display_startup_errors', '1');
error_reporting(E_ALL);

$log = new Logger('api');
//$log->pushHandler(new StreamHandler(__DIR__.'/var/logs/api.log'));

$url = $_GET['url'] ?? null;
$method = $_SERVER['REQUEST_METHOD'];
$headers = getallheaders();

$toUnset = [
    'Referer',
    'Cookie',
    'Upgrade-Insecure-Requests',
    'Accept-Encoding',
    'Cache-Control',
    'Mod-Rewrite',
    'Host',
];

foreach ($toUnset as $key) {
    unset($headers[$key]);
}

foreach ($headers as $key => $header) {
    if (str_starts_with($key, 'Sec-')) {
        unset($headers[$key]);
    }
}

//$headers['Content-Type'] = 'application/json';

$log->info('API request received', ['url' => $url, 'method' => $method, 'headers' => $headers]);
if ('POST' === $method) {
    $log->debug('POST', $_POST);
}

$client = HttpClient::create();

switch ($method) {
    case 'GET':
        unset($headers['Content-Length']);
        $response = $client->request(
            $method,
            $url,
            [
                'headers' => $headers,
            ]
        );
        break;

    case 'POST':
        $response = $client->request(
            $method,
            $url,
            [
                'headers' => $headers,
                'body' => file_get_contents('php://input'),
            ]
        );
        break;
}


$log->debug('Server responded', ['response-code' => $response->getStatusCode(), 'headers' => $response->getHeaders(false)]);

http_response_code($response->getStatusCode());
echo $response->getContent(false);