import Browser
import Browser.Navigation as Nav
import Browser.Events as Evts
import Html exposing (Html, div, span, text)
import Html.Attributes exposing (..)
import Html.Events exposing (onClick)

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

type Model = Int

init : () -> Url.Url -> Nav.Key -> (Model, Cmd Msg)

-- UPDATE

-- VIEW
