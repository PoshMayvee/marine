import fs from 'fs';
import path from 'path';
import download from 'download';
import { MarineService } from '../MarineService';
import { callAvm } from '@fluencelabs/avm';
import { JSONArray, JSONObject } from '../types';

const fsPromises = fs.promises;

const vmPeerId = '12D3KooWNzutuy8WHXDKFqFsATvCR6j9cj2FijYbnd47geRKaQZS';

const b = (s: string) => {
    return Buffer.from(s);
};

const loadWasmModule = async (waspPath: string) => {
    const fullPath = path.join(waspPath);
    const buffer = await fsPromises.readFile(fullPath);
    const module = await WebAssembly.compile(buffer);
    return module;
};

const redisDownloadUrl = 'https://github.com/fluencelabs/redis/releases/download/v0.15.0_w/redis.wasm';
const sqliteDownloadUrl = 'https://github.com/fluencelabs/sqlite/releases/download/v0.16.0_w/sqlite3.wasm';

const examplesDir = path.join(__dirname, '../../../../examples');

const dontLog = () => {};

describe('Fluence app service tests', () => {
    it('Testing greeting service', async () => {
        // arrange
        const marine = await loadWasmModule(path.join(__dirname, '../../dist/marine-js.wasm'));
        const greeting = await loadWasmModule(path.join(examplesDir, './greeting/artifacts/greeting.wasm'));

        const marineService = new MarineService(marine, greeting, 'srv', dontLog);
        await marineService.init();

        // act
        const res = marineService.call('greeting', ['test'], undefined);

        // assert
        expect(res).toBe('Hi, test');
    });

    it('Testing greeting service with object args', async () => {
        // arrange
        const marine = await loadWasmModule(path.join(__dirname, '../../dist/marine-js.wasm'));
        const greeting = await loadWasmModule(path.join(examplesDir, './greeting/artifacts/greeting.wasm'));

        const marineService = new MarineService(marine, greeting, 'srv', dontLog);
        await marineService.init();

        // act
        const res = marineService.call('greeting', { name: 'test' }, undefined);

        // assert
        expect(res).toBe('Hi, test');
    });

    it('Testing greeting service with records', async () => {
        // arrange
        const marine = await loadWasmModule(path.join(__dirname, '../../dist/marine-js.wasm'));
        const greeting = await loadWasmModule(
            path.join(examplesDir, './greeting_record/artifacts/greeting-record.wasm'),
        );

        const marineService = new MarineService(marine, greeting, 'srv', dontLog);
        await marineService.init();

        // act
        const greetingRecordResult = marineService.call('greeting_record', [], undefined);
        const voidResult: any = marineService.call('void_fn', [], undefined);

        // assert
        expect(greetingRecordResult).toMatchObject({
            str: 'Hello, world!',
            num: 42,
        });
        expect(voidResult).toStrictEqual(null);
    });

    it('Running avm through Marine infrastructure', async () => {
        // arrange
        const avmPackagePath = require.resolve('@fluencelabs/avm');
        const avm = await loadWasmModule(path.join(path.dirname(avmPackagePath), 'avm.wasm'));
        const marine = await loadWasmModule(path.join(__dirname, '../../dist/marine-js.wasm'));

        const testAvmInMarine = new MarineService(marine, avm, 'avm', dontLog);
        await testAvmInMarine.init();

        const s = `(seq
            (par 
                (call "${vmPeerId}" ("local_service_id" "local_fn_name") [] result_1)
                (call "remote_peer_id" ("service_id" "fn_name") [] g)
            )
            (call "${vmPeerId}" ("local_service_id" "local_fn_name") [] result_2)
        )`;

        // act
        const res = await callAvm(
            (args: JSONArray | JSONObject): unknown => testAvmInMarine.call('invoke', args, undefined),
            {
                currentPeerId: vmPeerId,
                initPeerId: vmPeerId,
                timestamp: Date.now(),
                ttl: 10000,
            },
            s,
            b(''),
            b(''),
            [],
        );
        await testAvmInMarine.terminate();

        // assertMarine
        expect(res).toMatchObject({
            retCode: 0,
            errorMessage: '',
        });
    });

    it('Testing sqlite wasm', async () => {
        jest.setTimeout(10000);
        const control = await loadWasmModule(path.join(__dirname, '../../dist/marine-js.wasm'));
        const buf = await download(sqliteDownloadUrl);
        const sqlite = await WebAssembly.compile(buf);

        const marine = new MarineService(control, sqlite, 'sqlite', dontLog);
        await marine.init();

        let result: any;

        result = marine.call('sqlite3_open_v2', [':memory:', 6, ''], undefined);
        const dbHandle = result.db_handle;
        result = marine.call(
            'sqlite3_exec',
            [dbHandle, 'CREATE VIRTUAL TABLE users USING FTS5(body)', 0, 0],
            undefined,
        );

        expect(result).toMatchObject({ err_msg: '', ret_code: 0 });

        result = marine.call(
            'sqlite3_exec',
            [dbHandle, "INSERT INTO users(body) VALUES('AB'), ('BC'), ('CD'), ('DE')", 0, 0],
            undefined,
        );

        expect(result).toMatchObject({ err_msg: '', ret_code: 0 });

        result = marine.call(
            'sqlite3_exec',
            [dbHandle, "SELECT * FROM users WHERE users MATCH 'A* OR B*'", 0, 0],
            undefined,
        );

        expect(result).toMatchObject({ err_msg: '', ret_code: 0 });
    });

    it.skip('Testing redis wasm', async () => {
        const control = await loadWasmModule(path.join(__dirname, '../../dist/marine-js.wasm'));
        const buf = await download(redisDownloadUrl);
        const redis = await WebAssembly.compile(buf);

        const marine = new MarineService(control, redis, 'redis', dontLog);
        await marine.init();

        const result1 = marine.call('invoke', ['SET A 10'], undefined);
        const result2 = marine.call('invoke', ['SADD B 20'], undefined);
        const result3 = marine.call('invoke', ['GET A'], undefined);
        const result4 = marine.call('invoke', ['SMEMBERS B'], undefined);
        const result5 = marine.call(
            'invoke',
            ["eval \"redis.call('incr', 'A') return redis.call('get', 'A') * 8 + 5\"  0"],
            undefined,
        );

        expect(result1).toBe('+OK\r\n');
        expect(result2).toBe(':1\r\n');
        expect(result3).toBe('$2\r\n10\r\n');
        expect(result4).toBe('*1\r\n$2\r\n20\r\n');
        expect(result5).toBe(':93\r\n');
    });

    it('Testing service which fails', async () => {
        // arrange
        const marine = await loadWasmModule(path.join(__dirname, '../../dist/marine-js.wasm'));
        const failing = await loadWasmModule(path.join(examplesDir, './failing/artifacts/failing.wasm'));

        const marineService = new MarineService(marine, failing, 'srv', dontLog);
        await await marineService.init();

        // act
        try {
            await marineService.call('failing', [], undefined);
            // should never succeed
            expect(true).toBe(false);
        } catch (e) {
            // assert
            expect(e).toBeInstanceOf(WebAssembly.RuntimeError);
            const re = e as WebAssembly.RuntimeError;
            expect(re.message).toBe('unreachable');
        }
    });

    it('Checking error when calling non-existent function', async () => {
        // arrange
        const marine = await loadWasmModule(path.join(__dirname, '../../dist/marine-js.wasm'));
        const greeting = await loadWasmModule(path.join(examplesDir, './failing/artifacts/failing.wasm'));

        const marineService = new MarineService(marine, greeting, 'srv', dontLog);
        await await marineService.init();

        // act
        try {
            await marineService.call('do_not_exist', [], undefined);
            // should never succeed
            expect(true).toBe(false);
        } catch (e) {
            // assert
            expect(e).toBeInstanceOf(Error);
            expect((e as Error).message).toBe(
                'marine-js failed with: Error calling module function: function with name `do_not_exist` is missing',
            );
        }
    });
});
