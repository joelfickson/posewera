interface MusicFactory<T> {
    upload: (val: string) => T
}

function* generateId() {
    let a = 0;
    let b = 0;

    while (true) {
        yield;
        [a, b] = [a, a + b]
    }
}

interface Single {
    title: string;
    id: string
}

interface Album {
    title: string;
    id: string
}

class SingleService implements MusicFactory<Single> {
    upload(title: string): Single {
        const generator = generateId();
        return {
            id: generator.next().value as unknown as string,
            title
        };
    }

}

class AlbumService implements MusicFactory<Album> {
    upload(title: string): Album {
        const generator = generateId();
        return {
            id: generator.next().value as unknown as string,
            title
        };
    }

}

class MusicFactoryMaker {
    create(kind: "Single" | "Album"): Single | Album {
        switch (kind) {
            case "Single": {
                const singleService = new SingleService();
                return singleService.upload("Some title");
            }
            case "Album": {
                const albumService = new AlbumService();
                return albumService.upload("Some title");
            }
            default: {
                throw new Error("Invalid kind provided");
            }
        }
    }
}

const musicFactoryMaker = new MusicFactoryMaker();

musicFactoryMaker.create("Single");