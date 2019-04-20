port module Main exposing (main)

import Html exposing (Html, text, div, button, input)
import Html.Events exposing (onClick, onInput)
import Html.Attributes exposing (type_, value)
import Browser
import Json.Encode exposing (Value)
import Json.Decode as Decode

main : Program () Model Msg
main =
    Browser.element
        { init = init
        , update = update
        , view = view
        , subscriptions = subscriptions
        }

type alias User = 
    { uid : Int
    , nickname : String
    , icon : List Int
    }

type NetMessage = 
    LoginResponse
        { users : List User }
    | LoginRequest
        { username : String
        , password : String
        }

type alias Packet = 
    { kind : Int
    , sequence : Int
    , flags : Int
    , message : NetMessage
    }

type alias ChatLine =
    { user : String
    , text : String
    }

type alias Model =
    { message : String
    , users : List User
    , chat : List ChatLine 
    }


type Msg = 
    SendNet String
    | UpdateStr String
    | Error String

init : () -> ( Model, Cmd Msg )
init _ = 
    ( { message = "Hello world"
        , users = []
        , chat = []
        }, Cmd.none )



port netWrite : String -> Cmd msg
port netRead : (Value -> msg) -> Sub msg

update : Msg -> Model -> ( Model, Cmd Msg )
update msg model = 
    case msg of
        SendNet str ->
            ( model, netWrite str )
        UpdateStr str ->
            ( { model | message = str }, Cmd.none )
        Error str ->
            (model, Cmd.none)

view : Model -> Html Msg
view model = 
    div []
        [ input [ type_ "text", onInput UpdateStr, value model.message ] []
        , div [] [ text model.message ]
        , button
            [ onClick (SendNet model.message) ]
            [ text "Send" ]
        ]

subscriptions : Model -> Sub Msg
subscriptions model = 
    netRead (decodeValue)

decodeValue : Value -> Msg
decodeValue x = 
    let
        result =
            Decode.decodeValue Decode.string x
    in
        case result of
            Ok string ->
                UpdateStr string
            Err _ ->
                Error "Got sosmething weird from JS"
            
