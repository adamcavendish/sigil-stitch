export abstract class BaseController {
  abstract handleRequest(req: Request): Response;

  protected log() {
    console.log('handled')
  }
}
