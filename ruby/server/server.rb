# frozen_string_literal: true

require 'sinatra'
require 'rimage'

get '/' do
   erb :"index.html"
end

post '/upload' do
  file = params[:file]
  converted_path = Rimage.png_to_jpg(file[:tempfile].path)
  base64_image = Base64.strict_encode64(File.read(converted_path))

  erb :"image.html", locals: { base64_image: base64_image, format: 'jpg' }
end
