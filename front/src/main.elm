import Browser
import Browser.Navigation as Nav
import Browser.Events as Evts
import Html exposing (Html, div, span, text)
import Html.Attributes exposing (..)
import Html.Events exposing (onClick)
import Url

-- MAIN

main =
    Browser.application
        { init = init
        , update = update
        , subscriptions = subscriptions
        , view = view
        , onUrlChange = UrlChanged
        , onUrlRequest = UrlRequested
        }

-- MODEL

type alias Model =
    { url : Url.Url
    , key : Nav.Key
    }

init : () -> Url.Url -> Nav.Key -> (Model, Cmd Msg)
init _ url key =
    (Model url key, Cmd.none)

-- UPDATE

type Msg
    = UrlRequested Browser.UrlRequest
    | UrlChanged Url.Url

update : Msg -> Model -> (Model, Cmd Msg)
update msg model =
    case msg of
        UrlRequested urlRequest ->
            case urlRequest of
                Browser.Internal url ->
                    (model, Nav.pushUrl model.key (Url.toString url))
                Browser.External href ->
                    (model, Nav.load href)
        UrlChanged url ->
            ({model | url = url}, Cmd.none)

-- SUBSCRIPTIONS

subscriptions : Model -> Sub Msg
subscriptions model =
    Sub.none

-- VIEW

view : Model -> Browser.Document Msg
view model =
    { title = "WebApp Manager"
    , body = [div [] [text "First element entirely in elm!"]]
    }
