import { IonContent, IonInput, IonButton, IonLabel } from '@ionic/react';
import React from 'react';

import { HelloPromiseClient } from '../proto/hello_grpc_web_pb'
import { EchoRequest } from '../proto/hello_pb'

const API_URL = process.env.REACT_APP_API_URL || ''
const client = new HelloPromiseClient(API_URL)


export const Hello: React.FC = () => {
  const [param, setParam] = React.useState<string | null | undefined>(undefined)
  const [req, setReq] = React.useState<EchoRequest | undefined>()
  const [res, setRes] = React.useState('')

  React.useEffect(() => {
    (async () => {
      if (!req) return

      const res = await client.echo(req)
      const text = `ret: ${res.getRet()}`
      setRes(text)
    })()
  }, [req, setRes])

  return (
    <IonContent>
      <IonInput value={param} onIonChange={(e) => setParam(e.detail.value)}></IonInput>
      <IonButton onClick={() => {
        if (!param) return

        const req = new EchoRequest()
        req.setParam(param)
        setReq(req)
      }}>Echo</IonButton>
      <IonLabel>{res}</IonLabel>
    </IonContent>
  )
}
