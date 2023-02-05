<?php

require 'php/Geekality/CrossOriginProxy.php';

Geekality\CrossOriginProxy::proxy([
    ['host' => 'elkato.de', 'scheme' => 'https'],
    ['host' => 'www.elkato.de', 'scheme' => 'https'],
]);